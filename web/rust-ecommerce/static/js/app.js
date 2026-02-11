// 전자상거래 시스템 JavaScript
document.addEventListener('DOMContentLoaded', function() {
    // 동적 행성 강조
    const forms = document.querySelectorAll('form');
    forms.forEach(form => {
        form.addEventListener('submit', function(e) {
            e.preventDefault();
            
            // 로딩 애니메이션 표시
            const button = form.querySelector('button[type="submit"]');
            if (button) {
                button.innerHTML = '처리 중...';
                button.disabled = true;
            }
            
            // 폼 데이터 수집
            const formData = new FormData(form);
            const data = {};
            for (let [key, value] of formData.entries()) {
                data[key] = value;
            }
            
            // API 호출 (임시)
            setTimeout(() => {
                // 성공 메시지
                alert('✅ 사용자가 성공적으로 생성되었습니다!');
                
                if (button) {
                    button.innerHTML = '✅ 생성 완료';
                    button.disabled = false;
                }
                
                // 폼 초기화
                form.reset();
            }, 1000);
        });
    });
    
    // 링크 hover 효과
    const links = document.querySelectorAll('a');
    links.forEach(link => {
        link.addEventListener('mouseenter', function() {
            this.style.transform = 'translateX(2px)';
        });
        
        link.addEventListener('mouseleave', function() {
            this.style.transform = 'translateX(0)';
        });
    });
});

// 테이블 행 호버 효과
function highlightRow(row) {
    row.style.backgroundColor = '#f0f8ff';
}

function unhighlightRow(row) {
    row.style.backgroundColor = '';
}

// 검색 기능 (미래에 추가할 기능)
function searchUsers() {
    const searchTerm = document.getElementById('search').value.toLowerCase();
    const rows = document.querySelectorAll('table tr');
    
    for (let i = 1; i < rows.length; i++) {
        const username = rows[i].cells[1]?.textContent.toLowerCase();
        const email = rows[i].cells[2]?.textContent.toLowerCase();
        
        if (username && username.includes(searchTerm) || 
            email && email.includes(searchTerm)) {
            rows[i].style.display = '';
        } else {
            rows[i].style.display = 'none';
        }
    }
}