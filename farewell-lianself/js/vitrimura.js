(function() {
    let intervalId = null;
    let shardsContainer = null;

    window.Vitrimura = {
        init: function() {
            createShards();
            startGlitchEffect();
        },
        cleanup: function(nextState) {
            stopGlitchEffect();
            removeShards(nextState);
        }
    };

    function createShards() {
        const originalWindow = document.querySelector('.window');
        if (!originalWindow) return;

        // Hide original window
        originalWindow.classList.add('window-hidden');

        // Create container
        shardsContainer = document.createElement('div');
        shardsContainer.className = 'shards-container';
        
        // Create 3 shards
        // We clone the window content for each shard
        for (let i = 1; i <= 3; i++) {
            const shard = document.createElement('div');
            shard.className = `shard shard-${i}`;
            
            // Clone the window
            const windowClone = originalWindow.cloneNode(true);
            windowClone.classList.remove('window-hidden'); // Ensure clone is visible
            windowClone.classList.add('window-clone');
            
            // Remove ID to avoid duplicates if any (though we use classes)
            windowClone.removeAttribute('id');
            
            shard.appendChild(windowClone);
            shardsContainer.appendChild(shard);
        }

        document.body.appendChild(shardsContainer);

        // Trigger reflow to ensure transitions work if we add them later
        shardsContainer.offsetHeight;
    }

    function removeShards(nextState) {
        const originalWindow = document.querySelector('.window');
        
        if (shardsContainer) {
            if (nextState === 'NOSTOFOBIA') {
                // Instant removal
                shardsContainer.remove();
                shardsContainer = null;
                if (originalWindow) originalWindow.classList.remove('window-hidden');
            } else {
                // Reassemble logic (Piece back together)

                // Clear content in shards to avoid visual clash
                shardsContainer.querySelectorAll('.code-area').forEach(el => {
                    el.style.opacity = '0';
                    // Instant or very fast fade out
                    el.style.transition = 'opacity 0.05s'; 
                });

                // 1. Freeze shards in current position
                const shards = shardsContainer.querySelectorAll('.shard');
                shards.forEach(shard => {
                    const computedStyle = window.getComputedStyle(shard);
                    const transform = computedStyle.transform;
                    shard.style.transform = transform;
                    shard.style.animation = 'none';
                });

                // Force reflow
                shardsContainer.offsetHeight;

                // 2. Add transition class and reset transform to origin
                shardsContainer.classList.add('reassembling');
                shards.forEach(shard => {
                    // We need to set style.transform to empty or identity to let CSS take over or just set it here
                    // Since we set inline style above, we must clear it or set it to identity
                    // But we want a transition.
                    // Let's set it to identity with a transition.
                    shard.style.transition = 'transform 0.3s cubic-bezier(0.4, 0, 0.2, 1)';
                    shard.style.transform = 'translate(0, 0) rotate(0deg)';
                });
                
                // Wait for reassembly animation (very fast)
                setTimeout(() => {
                    if (shardsContainer) {
                        shardsContainer.remove();
                        shardsContainer = null;
                    }
                    if (originalWindow) {
                        originalWindow.classList.remove('window-hidden');
                    }
                }, 300); // 300ms match CSS transition
            }
        } else {
            if (originalWindow) originalWindow.classList.remove('window-hidden');
        }
    }

    function startGlitchEffect() {
        if (intervalId) return;
        triggerGlitch();
        intervalId = setInterval(triggerGlitch, 100); 
    }

    function triggerGlitch() {
        // We need to target line numbers inside the SHARDS now, because the original window is hidden
        const visibleLineNums = Array.from(document.querySelectorAll('.shard .line-num'));
        if (visibleLineNums.length === 0) return;

        const idleNums = visibleLineNums.filter(el => !el.classList.contains('glitch-active') && !el.classList.contains('glitch-small'));
        
        if (idleNums.length > 0) {
            const target = idleNums[Math.floor(Math.random() * idleNums.length)];
            const isViolent = Math.random() < 0.2;
            
            if (isViolent) {
                target.classList.add('glitch-active');
                setTimeout(() => {
                    if (document.body.contains(target)) target.classList.remove('glitch-active');
                }, 500);
            } else {
                target.classList.add('glitch-small');
                setTimeout(() => {
                    if (document.body.contains(target)) target.classList.remove('glitch-small');
                }, 200);
            }
        }
    }

    function stopGlitchEffect() {
        if (intervalId) {
            clearInterval(intervalId);
            intervalId = null;
        }
    }

})();
