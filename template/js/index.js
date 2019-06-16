let imp = import('../pkg/index.js');
let wasm_sha1;

imp
  .then((wasm) => {
    wasm_sha1 = wasm;
    let sha1 = wasm_sha1.Sha1sum.new();

    var changeFile = function(e) {
      const CHUNK_SIZE = 100 * 1024 * 1024;
      let offset = 0;

      let file = e.target.files[0];
      let reader = new FileReader();

      reader.onload = function() {
        let buf = new Uint8Array(reader.result);
        sha1.update(buf);
        offset += CHUNK_SIZE;
        seek();
      };

      reader.onerror = function(e) {
        console.log(e);
      };

      let seek = function() {
        if (offset >= file.size) {
          let result = sha1.result();
          console.log(result);
        } else {
          let slice = file.slice(offset, offset + CHUNK_SIZE);
          reader.readAsArrayBuffer(slice);
        }
      };
      seek();
    };
    document.addEventListener('change', changeFile, false);
  })
  .catch(console.error);
