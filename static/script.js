document.addEventListener('DOMContentLoaded', function() {
  document.querySelectorAll('.post-content .code-block').forEach(function(block) {
    var btn = document.createElement('button');
    btn.className = 'copy-btn';
    btn.textContent = 'Copy';
    btn.addEventListener('click', function() {
      var lcs = block.querySelectorAll('.lc');
      var text = Array.from(lcs).map(function(l) { return l.textContent; }).join('\n');
      if (!text) text = block.textContent.replace(/^\s*\d+\s*/gm, '');
      navigator.clipboard.writeText(text).then(function() {
        btn.textContent = 'Copied!';
        setTimeout(function() { btn.textContent = 'Copy'; }, 2000);
      }).catch(function() {
        btn.textContent = 'Failed';
        setTimeout(function() { btn.textContent = 'Copy'; }, 2000);
      });
    });
    block.appendChild(btn);
  });

  var searchInput = document.getElementById('search-input');
  var prevLink = document.querySelector('a[data-nav="prev"]');
  var nextLink = document.querySelector('a[data-nav="next"]');

  document.addEventListener('keydown', function(e) {
    if (e.target.closest('input, textarea, select, [contenteditable]')) return;
    if (e.key === '/' && !e.ctrlKey && !e.metaKey && document.activeElement !== searchInput) {
      e.preventDefault();
      searchInput.focus();
    }
    if (e.key === 'ArrowLeft' && prevLink) { e.preventDefault(); window.location.href = prevLink.href; }
    if (e.key === 'ArrowRight' && nextLink) { e.preventDefault(); window.location.href = nextLink.href; }
  });
});
