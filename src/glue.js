(() => {
  if (typeof butterflySoulEngine === "undefined") {
    const control = {
      click: [undefined, undefined],
      mouse: [0, 0],
      keys: new Map(),

      element: undefined,
      el_size: [0,0],

      get() {
        return {
          click: this.click,
          mouse: this.mouse,
          keys: this.keys,
        };
      },

      getRelPos(e) {
        const el_w = this.el_size[0] / 2;
        const el_h = this.el_size[1] / 2;
        let x = (e.offsetX - el_w) / el_h;
        let y = (e.offsetY - el_h) / el_h;
        return [x, -y];
      },

      keyCallback(press, e) {
        this.keys[press ? "set" : "delete"](
          e.code,
          press && {
            alt: e.altKey,
            ctrl: e.ctrlKey,
            meta: e.metaKey,
            shift: e.shiftKey,
            repeat: e.repeat,
          }
        );
      },

      clickCallback(leftClick, e) {
        this.click[leftClick ? 0 : 1] = this.getRelPos(e);
        if (!leftClick) e.preventDefault();
      },

      mousemoveCallback(e) {
        this.mouse = this.getRelPos(e);
      },
    };
    control.getRelPos = control.getRelPos.bind(control);
    control.mousemoveCallback = control.mousemoveCallback.bind(control);
    control.keydownCallback = control.keyCallback.bind(control, true);
    control.keyupCallback = control.keyCallback.bind(control, false);
    control.mousedownCallback = control.clickCallback.bind(control, true);
    control.contextmenuCallback = control.clickCallback.bind(control, false);
    control.resize = () => {
      const el = control.element;
      control.el_size = [el.offsetWidth, el.offsetHeight];
      el.height = el.offsetHeight;
      el.width  = el.offsetWidth;
    };
    control.resizeObserver = new ResizeObserver(control.resize)

    butterflySoulEngine = {
      mount_control(el) {
        const oldEl = control.element;
        control.element = el;
        control.resize();

        if (oldEl) {
          control.resizeObserver.unobserve(oldEl)
          oldEl.removeEventListener("mousemove", control.mousemoveCallback);
          oldEl.removeEventListener("keydown", control.keydownCallback);
          oldEl.removeEventListener("keyup", control.keyupCallback);
          oldEl.removeEventListener("mousedown", control.mousedownCallback);
          oldEl.removeEventListener("contextmenu", control.contextmenuCallback);
          oldEl.removeEventListener("resize", control.resize);
        }

        control.resizeObserver.observe(el)
        const passived = { passive: false };
        el.addEventListener("mousemove", control.mousemoveCallback, passived);
        el.addEventListener("keydown", control.keydownCallback, passived);
        el.addEventListener("keyup", control.keyupCallback, passived);
        el.addEventListener("mousedown", control.mousedownCallback, passived);
        el.addEventListener(
          "contextmenu",
          control.contextmenuCallback,
          passived
        );
        el.tabIndex = 0
        el.focus()

        //init
        control.click = [undefined, undefined];
        control.mouse = [0, 0];
        control.keys = new Map();
      },
      control() {
        const result = control.get();
        control.click = [undefined, undefined];
        return result;
      },
    };
  }
})();
