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

  var aboutName = document.getElementById('about-name');
  if (aboutName) {
    aboutName.addEventListener('click', function() {
      if (this.querySelector('input')) return;
      var current = this.textContent.trim();
      var input = document.createElement('input');
      input.type = 'text';
      input.value = current;
      input.id = 'name-editor';
      input.style.cssText = 'font-size:1.6rem;font-weight:800;text-align:center;border:2px solid var(--primary);border-radius:8px;padding:4px 12px;font-family:inherit;color:var(--text);background:var(--bg);outline:none;width:240px;display:inline-block';
      this.textContent = '';
      this.appendChild(input);
      input.focus();
      input.select();

      function done() {
        var val = input.value.trim();
        if (val && val !== current) {
          fetch('/api/about', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ author_name: val })
          }).then(function(r) {
            aboutName.textContent = r.ok ? val : current;
          }).catch(function() {
            aboutName.textContent = current;
          });
        } else {
          aboutName.textContent = current;
        }
      }

      input.addEventListener('blur', done);
      input.addEventListener('keydown', function(ev) {
        if (ev.key === 'Enter') { ev.preventDefault(); input.blur(); }
        if (ev.key === 'Escape') { ev.preventDefault(); aboutName.textContent = current; }
      });
    });
  }

  var aboutAvatar = document.getElementById('about-avatar');
  var avatarInput = document.getElementById('avatar-file-input');
  var cropModal = document.getElementById('crop-modal');
  var cropContainer = document.getElementById('crop-container');
  var cropConfirm = document.getElementById('crop-confirm');
  var cropCancel = document.getElementById('crop-cancel');

  if (aboutAvatar && avatarInput && cropModal) {
    aboutAvatar.style.cursor = 'pointer';
    aboutAvatar.addEventListener('click', function() {
      avatarInput.click();
    });

    var cropper = null;

    avatarInput.addEventListener('change', function() {
      var file = this.files[0];
      if (!file) return;
      this.value = '';

      var reader = new FileReader();
      reader.onload = function(e) {
        cropContainer.innerHTML = '<img id="crop-image" src="' + e.target.result + '" style="max-width:100%">';
        cropModal.style.display = 'flex';

        var img = document.getElementById('crop-image');
        img.onload = function() {
          if (cropper) cropper.destroy();
          cropper = new Cropper(img, {
            aspectRatio: 1,
            viewMode: 1,
            autoCropArea: 1,
            movable: true,
            zoomable: true,
            rotatable: false,
          });
        };
      };
      reader.readAsDataURL(file);
    });

    cropConfirm.addEventListener('click', function() {
      if (!cropper) return;
      cropConfirm.disabled = true;
      cropConfirm.textContent = '上传中...';
      cropper.getCroppedCanvas({ width: 400, height: 400 }).toBlob(function(blob) {
        var formData = new FormData();
        formData.append('avatar', blob, 'avatar.png');
        fetch('/api/upload-avatar', {
          method: 'POST',
          body: formData
        }).then(function(r) { return r.json(); }).then(function(data) {
          if (data.avatar_path) {
            aboutAvatar.src = data.avatar_path + '?t=' + Date.now();
          }
        }).catch(function() {});
        cropModal.style.display = 'none';
        if (cropper) { cropper.destroy(); cropper = null; }
        cropConfirm.disabled = false;
        cropConfirm.textContent = '确认';
      });
    });

    cropCancel.addEventListener('click', function() {
      cropModal.style.display = 'none';
      if (cropper) { cropper.destroy(); cropper = null; }
    });

    cropModal.addEventListener('click', function(e) {
      if (e.target === cropModal) {
        cropModal.style.display = 'none';
        if (cropper) { cropper.destroy(); cropper = null; }
      }
    });
  }
});
