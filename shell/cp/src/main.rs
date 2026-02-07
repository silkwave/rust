use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

// 복사 옵션을 저장하는 구조체
struct CopyOptions {
    recursive: bool, // 디렉토리 재귀적 복사
    force: bool,     // 기존 파일 강제 덮어쓰기
    preserve: bool,  // 파일 속성 보존
}

fn main() -> io::Result<()> {
    // 명령행 인자 가져오기
    let args: Vec<String> = env::args().collect();

    // 최소 인자 수 확인 (프로그램 이름 + 소스 + 목적지)
    if args.len() < 3 {
        eprintln!("사용법: cp [옵션] <소스> <목적지>");
        eprintln!("옵션:");
        eprintln!("  -r, --recursive  디렉토리를 재귀적으로 복사");
        eprintln!("  -f, --force      기존 파일을 덮어쓰기");
        eprintln!("  -p, --preserve    파일 속성 보존");
        std::process::exit(1);
    }

    // 기본 옵션 설정
    let mut options = CopyOptions {
        recursive: false,
        force: false,
        preserve: false,
    };

    // 파일 인자들을 저장할 벡터
    let mut file_args = Vec::new();

    // 옵션 파싱 (마지막 인자는 목적지이므로 제외)
    for arg in &args[1..args.len() - 1] {
        match arg.as_str() {
            "-r" | "--recursive" => options.recursive = true,
            "-f" | "--force" => options.force = true,
            "-p" | "--preserve" => options.preserve = true,
            _ => file_args.push(arg.clone()),
        }
    }

    // 목적지 경로
    let destination = args.last().unwrap().clone();

    // 단일 파일 복사 또는 다중 파일 복사
    if file_args.len() == 1 {
        let source = &file_args[0];
        copy_path(source, &destination, &options)?;
    } else {
        copy_multiple_files(&file_args, &destination, &options)?;
    }

    Ok(())
}

// 단일 경로(파일 또는 디렉토리) 복사
fn copy_path(source: &str, destination: &str, options: &CopyOptions) -> io::Result<()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    // 소스 파일 존재 확인
    if !source_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("소스 '{}'가 존재하지 않습니다", source),
        ));
    }

    // 디렉토리인 경우 재귀 옵션 확인
    if source_path.is_dir() {
        if !options.recursive {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("디렉토리 '{}'를 복사하려면 -r 옵션이 필요합니다", source),
            ));
        }
        copy_directory(source_path, dest_path, options)?;
    } else {
        // 파일인 경우 파일 복사
        copy_file(source_path, dest_path, options)?;
    }

    Ok(())
}

// 여러 파일을 목적지 디렉토리로 복사
fn copy_multiple_files(
    sources: &[String],
    destination: &str,
    options: &CopyOptions,
) -> io::Result<()> {
    let dest_path = Path::new(destination);

    // 목적지가 없으면 디렉토리 생성
    if !dest_path.exists() {
        fs::create_dir_all(dest_path)?;
    } else if !dest_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "여러 파일을 복사할 때 목적지는 디렉토리여야 합니다",
        ));
    }

    // 각 소스 파일을 목적지 디렉토리로 복사
    for source in sources {
        let source_path = Path::new(source);
        let file_name = source_path.file_name().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{}'의 파일 이름을 확인할 수 없습니다", source),
            )
        })?;

        // 목적지 경로 조합 (목적지 디렉토리 + 파일 이름)
        let final_dest = dest_path.join(file_name);

        // 디렉토리인 경우 재귀 옵션 확인
        if source_path.is_dir() {
            if !options.recursive {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("디렉토리 '{}'를 복사하려면 -r 옵션이 필요합니다", source),
                ));
            }
            copy_directory(source_path, &final_dest, options)?;
        } else {
            copy_file(source_path, &final_dest, options)?;
        }
    }

    Ok(())
}

// 단일 파일 복사
fn copy_file(source: &Path, destination: &Path, options: &CopyOptions) -> io::Result<()> {
    // 목적지 파일이 존재하고 강제 옵션이 없으면 에러
    if destination.exists() && !options.force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "목적지 '{}'가 이미 존재합니다 (덮어쓰려면 -f 옵션 사용)",
                destination.display()
            ),
        ));
    }

    // 목적지의 부모 디렉토리 생성
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    // 파일 복사 수행
    let mut source_file = File::open(source)?;
    let mut dest_file = File::create(destination)?;

    // 8KB 버퍼로 효율적으로 복사
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = source_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // 파일 끝에 도달
        }
        dest_file.write_all(&buffer[..bytes_read])?;
    }

    // 파일 속성 보존 옵션이 있는 경우
    if options.preserve {
        preserve_attributes(source, destination)?;
    }

    Ok(())
}

// 디렉토리 재귀적 복사
fn copy_directory(source: &Path, destination: &Path, options: &CopyOptions) -> io::Result<()> {
    // 목적지가 존재하는 경우 확인
    if destination.exists() {
        if destination.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "디렉토리를 파일로 복사할 수 없습니다",
            ));
        }
    } else {
        // 목적지 디렉토리 생성
        fs::create_dir_all(destination)?;
    }

    // 디렉토리 내용물 순회하며 복사
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let file_name = source_path.file_name().unwrap();
        let dest_path = destination.join(file_name);

        if source_path.is_dir() {
            // 하위 디렉토리 재귀적 복사
            copy_directory(&source_path, &dest_path, options)?;
        } else {
            // 파일 복사
            copy_file(&source_path, &dest_path, options)?;
        }
    }

    // 파일 속성 보존 옵션이 있는 경우
    if options.preserve {
        preserve_attributes(source, destination)?;
    }

    Ok(())
}

// 파일 속성 보존 (Unix 시스템에서 권한 비트 보존)
fn preserve_attributes(source: &Path, destination: &Path) -> io::Result<()> {
    let metadata = fs::metadata(source)?;

    // Unix 시스템에서만 실행
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // 파일 권한 비트 보존
        let mode = metadata.permissions().mode();
        let mut perms = fs::metadata(destination)?.permissions();
        perms.set_mode(mode);
        fs::set_permissions(destination, perms)?;
    }

    Ok(())
}
