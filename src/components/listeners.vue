<script>
    /* eslint-disable no-useless-escape */
    
    const fs = window.__TAURI__.fs;
    const dialog = window.__TAURI__.dialog;
    const Window = window.__TAURI__.window;
    const event = window.__TAURI__.event;
    const invoke = window.__TAURI__.invoke;
    const shell = window.__TAURI__.shell;
    const { textareaFormatter } = require("./modules/textareaFormat")
    const { Dissect } = require("./modules/dissect")
    
    window.addEventListener("load", async function () {
      invoke('show_main_window');
    
      let savedFileLocation;
    
      let dissect = new Dissect({
        whitespace: /\s+/,
        number: /0x[\dA-Fa-f]+|-?(\d+\.?\d*|\.\d+)|#[\dA-Fa-f]{3,6}/,
        comment: /\/\*([^\*]|[^\/])*(\*\/?)?|(\/\/|#|; )[^\r\n]*/,
        string: /"(\\.|[^"\r\n])*"?|'(\\.|[^'\r\n])*'?/,
        keyword: /(div|and|as|case|catch|class|const|def|delete|die|do|else|elseif|esac|exit|extends|false|fi|finally|for|foreach|function|global|if|new|null|or|private|protected|public|published|resource|return|self|static|struct|switch|then|this|throw|true|try|var|void|while|xor|import|async)(?!\w|=)/,
        variable: /[\$\%\@](\->|\w)+(?!\w)|\${\w*}?/,
        define: /[$A-Z_a-z0-9]+/,
        op: /[\+\-\/=<>!]=?|[\(\)\{\}\[\]\.\|]/,
        other: /\S/,
      });
      let formattedText = new textareaFormatter(document.getElementById("viewer"), dissect);
      document.getElementById("viewer").focus()
    
      invoke('get_input').then(async (args) => {
        if (args[1]) {
          let readFile = await fs.readTextFile(args[1]);
          document.getElementById("viewer").value = `${readFile}`;
          formattedText.update()
          document.getElementById("title").innerHTML = `${args[1].replace(/^.*[\\\/]/, '')}`
          savedFileLocation = args[1]
        }
      })
    
      document.addEventListener('keydown', async function (e) {
        if (e.ctrlKey && e.key.toLowerCase() === 's') {
          e.preventDefault();
          if (!savedFileLocation) {
            const filePath = await dialog.save({
              multiple: false,
            });
    
            await fs.writeTextFile(filePath, document.getElementById("viewer").value)
            document.getElementById("title").innerHTML = `${filePath.replace(/^.*[\\\/]/, '')}`
          } else {
            await fs.writeTextFile(savedFileLocation, document.getElementById("viewer").value)
            document.getElementById("title").innerHTML = `${savedFileLocation.replace(/^.*[\\\/]/, '')}`
          }
    
        }
    
        document.getElementById("viewer").addEventListener("input", function () {
          if (!document.getElementById("title").innerHTML.includes(" *")) {
            document.getElementById("title").insertAdjacentHTML('beforeend', " *")
          }
        });
    
        if (e.key == 'Tab') {
          e.preventDefault();
        }
    
        if (e.ctrlKey && e.key === '=') {
          e.preventDefault();
          let getSize = window.getComputedStyle(document.getElementById("pre"), null).getPropertyValue("font-size");
          document.getElementById("pre").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) + 1 + "px" + " !important;"}`
          document.getElementById("viewer").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) + 1 + "px" + " !important;"}`
        }
    
        if (e.ctrlKey && e.key === '-') {
          e.preventDefault();
          let getSize = window.getComputedStyle(document.getElementById("pre"), null).getPropertyValue("font-size");
          document.getElementById("pre").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) - 1 + "px" + " !important;"}`
          document.getElementById("viewer").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) - 1 + "px" + " !important;"}`
        }
    
        if (e.ctrlKey && e.key === "r") {
          window.location.reload();
        }
      });
    
      document.getElementById("viewer").addEventListener("dragenter", async function (e) {
        e.preventDefault();
    
        event.listen('tauri://file-drop', async event => {
          try {
            let readFile = await fs.readTextFile(event.payload[0]);
            document.getElementById("viewer").value = `${readFile}`;
            formattedText.update()
            await Window.appWindow.setFocus()
          } catch (err) {
            alert(err)
          }
        })
      })
      document.getElementById("viewer").addEventListener("click", async function () {
        let x = document.getElementById("fileSubCategory");
        let a = document.getElementById("viewSubCategory");
        let e = document.getElementById("editSubCategory");
        x.style.cssText = ";display:none !important;";
        a.style.cssText = ";display:none !important;";
        e.style.cssText = ";display:none !important;";
      })
    
      document.getElementById("file").addEventListener('click', async function () {
        let x = document.getElementById("fileSubCategory");
        let a = document.getElementById("viewSubCategory");
        let e = document.getElementById("editSubCategory");
        let display = window.getComputedStyle(x, null).getPropertyValue("display");
        if (display === "none") {
          x.style.cssText = ";display:block !important;";
          a.style.cssText = ";display:none !important;";
          e.style.cssText = ";display:none !important;";
        } else {
          x.style.cssText = ";display:none !important;";
        }
      })
    
      document.getElementById("edit").addEventListener('click', async function () {
        let x = document.getElementById("editSubCategory");
        let a = document.getElementById("viewSubCategory");
        let e = document.getElementById("fileSubCategory");
        let display = window.getComputedStyle(x, null).getPropertyValue("display");
        if (display === "none") {
          x.style.cssText = ";display:block !important;";
          a.style.cssText = ";display:none !important;";
          e.style.cssText = ";display:none !important;";
        } else {
          x.style.cssText = ";display:none !important;";
        }
      })
    
      document.getElementById("view").addEventListener('click', async function () {
        let x = document.getElementById("viewSubCategory");
        let a = document.getElementById("editSubCategory");
        let e = document.getElementById("fileSubCategory");
        let display = window.getComputedStyle(x, null).getPropertyValue("display");
        if (display === "none") {
          x.style.cssText = ";display:block !important;";
          a.style.cssText = ";display:none !important;";
          e.style.cssText = ";display:none !important;";
        } else {
          x.style.cssText = ";display:none !important;";
        }
      })
    
      document.getElementById("newFile").addEventListener('click', async function () {
        savedFileLocation = "";
        document.getElementById("viewer").value = "";
        formattedText.update()
        document.getElementById("fileSubCategory").style.cssText = ";display:none !important;";
        document.getElementById("title").innerHTML = `Untitled`
      })
    
      document.getElementById("openFile").addEventListener('click', async function () {
        const selected = await dialog.open({
          multiple: false,
        });
    
        if (!selected) return;
        savedFileLocation = selected;
        try {
          let readFile = await fs.readTextFile(selected);
          document.getElementById("viewer").value = `${readFile}`;
          formattedText.update()
          document.getElementById("fileSubCategory").style.cssText = ";display:none !important;";
          document.getElementById("title").innerHTML = `${selected.replace(/^.*[\\\/]/, '')}`
        } catch (err) {
          alert(err);
        }
      })
    
      document.getElementById("saveFile").addEventListener('click', async function () {
        if (!savedFileLocation) {
          const filePath = await dialog.save({
            multiple: false,
          });
    
          await fs.writeTextFile(filePath, document.getElementById("viewer").value)
          document.getElementById("fileSubCategory").style.cssText = ";display:none !important;";
          savedFileLocation = filePath
          document.getElementById("title").innerHTML = `${filePath.replace(/^.*[\\\/]/, '')}`
        } else {
          await fs.writeTextFile(savedFileLocation, document.getElementById("viewer").value)
          document.getElementById("fileSubCategory").style.cssText = ";display:none !important;";
          document.getElementById("title").innerHTML = `${savedFileLocation.replace(/^.*[\\\/]/, '')}`
        }
      })
    
      document.getElementById("saveFileAs").addEventListener('click', async function () {
        const filePath = await dialog.save({
          multiple: false,
        });
    
        await fs.writeTextFile(filePath, document.getElementById("viewer").value)
        document.getElementById("fileSubCategory").style.cssText = ";display:none !important;";
        document.getElementById("title").innerHTML = `${filePath.replace(/^.*[\\\/]/, '')}`
      })
    
      document.getElementById("zoomIn").addEventListener('click', async function () {
        let getSize = window.getComputedStyle(document.getElementById("pre"), null).getPropertyValue("font-size");
        document.getElementById("pre").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) + 2 + "px" + " !important;"}`
        document.getElementById("viewer").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) + 2 + "px" + " !important;"}`
      })
      document.getElementById("zoomOut").addEventListener('click', async function () {
        let getSize = window.getComputedStyle(document.getElementById("pre"), null).getPropertyValue("font-size");
        document.getElementById("pre").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) - 2 + "px" + " !important;"}`
        document.getElementById("viewer").style.cssText = `font-size: ${parseInt(getSize.replace("px", "")) - 2 + "px" + " !important;"}`
      })
    
      document.getElementById("Info").addEventListener('click', async function () {
        await shell.open('https://github.com/GhostyORG/Nexus');
      })
    });
    </script>