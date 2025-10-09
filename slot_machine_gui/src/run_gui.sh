#!/bin/bash
set -e

# Xvfb가 이미 떠 있으면 종료
if pgrep Xvfb >/dev/null; then
  echo "🟡 Existing Xvfb detected — stopping old instance..."
  pkill Xvfb || true
fi

# Xvfb 실행
echo "🚀 Starting Xvfb on :99"
Xvfb :99 -screen 0 1024x768x24 &
sleep 1

# DISPLAY 설정
export DISPLAY=:99
echo "🖥️  DISPLAY set to $DISPLAY"

# Rust GUI 실행
cargo run



// # 1️⃣ 패키지 업데이트 및 Xvfb 설치
// sudo apt update && sudo apt install -y xvfb

// # 2️⃣ Xvfb가 이미 실행 중인지 확인
// ps -ef | grep Xvfb

// # 3️⃣ 만약 실행 중이라면 중복 방지를 위해 종료
// # (PID 예: 1234)
// # kill -9 1234

// # 4️⃣ 백그라운드로 가상 X 서버 실행
// Xvfb :99 -screen 0 1024x768x24 &

// # 5️⃣ DISPLAY 환경 변수 설정
// export DISPLAY=:99

// # 6️⃣ Rust GUI 앱 실행
// cargo run
