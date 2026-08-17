document.addEventListener('DOMContentLoaded', () => {
    const codeArea = document.querySelector('.code-area');
    const filenameEl = document.querySelector('.filename');
    const buttons = {
        close: document.querySelector('.close'),
        minimize: document.querySelector('.minimize'),
        maximize: document.querySelector('.maximize')
    };
    const windowEl = document.querySelector('.window');

    let currentState = 'DEFAULT';
    let nostofobiaInterval = null;

    // Initialize
    renderCode('DEFAULT');

    // Event Delegation for Buttons (to support cloned buttons in shards)
    document.addEventListener('click', (e) => {
        // Close Button -> Aurora
        if (e.target.matches('.btn.close')) {
            e.stopPropagation();
            switchState('AURORA');
        }
        // Minimize Button -> Vitrimura
        else if (e.target.matches('.btn.minimize')) {
            e.stopPropagation();
            switchState('VITRIMURA');
        }
        // Maximize Button -> Nostofobia
        else if (e.target.matches('.btn.maximize')) {
            e.stopPropagation();
            switchState('NOSTOFOBIA');
        }
        // Click outside window -> Default
        else if (currentState !== 'DEFAULT') {
            // If clicking inside the window (real or shards), do nothing
            if (!e.target.closest('.window') && !e.target.closest('.shard')) {
                switchState('DEFAULT');
            }
        }
    });

    function switchState(newState) {
        if (currentState === newState) {
            setState('DEFAULT');
        } else {
            setState(newState);
        }
    }

    function setState(state) {
        const oldState = currentState;
        currentState = state;

        // Handle Vitrimura Transitions
        if (oldState === 'VITRIMURA' && window.Vitrimura) {
            window.Vitrimura.cleanup(state);
        }
        
        // Update Body Class
        document.body.className = ''; // Clear all
        if (state !== 'DEFAULT') {
            document.body.classList.add(`state-${state.toLowerCase()}`);
        }

        // Update Filename
        filenameEl.textContent = STATE_NAMES[state];

        // Render Code
        // If coming from Vitrimura (and not to Nostofobia), delay content appearance
        if (oldState === 'VITRIMURA' && state !== 'NOSTOFOBIA') {
            codeArea.classList.add('content-delayed');
            renderCode(state);
            // 300ms reassembly + 1000ms delay = 1300ms
            setTimeout(() => {
                codeArea.classList.remove('content-delayed');
            }, 1300);
        } else {
            codeArea.classList.remove('content-delayed');
            renderCode(state);
        }

        // Handle Special Logic
        clearInterval(nostofobiaInterval);
        if (state === 'NOSTOFOBIA') {
            startNostofobiaEffect();
        } else if (state === 'VITRIMURA' && window.Vitrimura) {
            window.Vitrimura.init();
        }
    }

    function renderCode(state) {
        const data = CODE_DATA[state];
        codeArea.innerHTML = '';

        data.forEach((lineData, index) => {
            const lineDiv = document.createElement('div');
            lineDiv.className = 'code-line';
            lineDiv.setAttribute('data-source', lineData.source);

            // Line Number
            const lineNum = document.createElement('span');
            lineNum.className = 'line-num';
            lineNum.textContent = lineData.line;
            lineDiv.appendChild(lineNum);

            // Content
            // For Nostofobia, we want character level chaos.
            // For Vitrimura, we rely on CSS animations on spans.
            
            let contentHtml = lineData.html;
            
            if (state === 'NOSTOFOBIA') {
                contentHtml = chaosify(contentHtml);
            }

            // Append content (using a wrapper to keep line num separate if needed, but current CSS expects inline)
            // The original HTML structure was: <div class="code-line"> <span class="line-num">1</span> ...content... </div>
            // So we append the HTML string after the lineNum element.
            lineDiv.insertAdjacentHTML('beforeend', contentHtml);

            codeArea.appendChild(lineDiv);
        });
    }

    function chaosify(html) {
        // Strip tags to get text, then wrap every character? 
        // But we want to keep some structure or it looks like garbage.
        // The requirement says "High chaos, character level".
        // Let's try to parse the HTML, and for every text node, split it into spans.
        
        const tempDiv = document.createElement('div');
        tempDiv.innerHTML = html;
        
        function traverseAndSplit(node) {
            if (node.nodeType === Node.TEXT_NODE) {
                const text = node.textContent;
                if (text.trim() === '') return; // Skip whitespace only nodes if we want
                
                const fragment = document.createDocumentFragment();
                for (let char of text) {
                    const span = document.createElement('span');
                    span.textContent = char;
                    // Random inline styles for chaos
                    const r = () => Math.random() * 10 - 5;
                    span.style.display = 'inline-block';
                    span.style.transform = `translate(${r()}px, ${r()}px) rotate(${r()*5}deg)`;
                    // Removed random color to allow CSS control
                    // span.style.color = `hsl(${Math.random() * 360}, 70%, 60%)`;
                    fragment.appendChild(span);
                }
                node.parentNode.replaceChild(fragment, node);
            } else if (node.nodeType === Node.ELEMENT_NODE) {
                // Recursively handle children
                Array.from(node.childNodes).forEach(traverseAndSplit);
            }
        }
        
        Array.from(tempDiv.childNodes).forEach(traverseAndSplit);
        return tempDiv.innerHTML;
    }

    function startNostofobiaEffect() {
        // Randomly hide line numbers every 3 seconds
        const updateLineNums = () => {
            const lineNums = document.querySelectorAll('.line-num');
            lineNums.forEach(el => {
                el.style.visibility = Math.random() > 0.3 ? 'visible' : 'hidden';
            });
        };
        
        updateLineNums();
        nostofobiaInterval = setInterval(updateLineNums, 3000);
    }
});
