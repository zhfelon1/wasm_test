import * as wasm from "wasm_test";

// const canvas = document.getElementById("main-canvas");
// canvas.width = window.innerWidth - 60;
// canvas.height = 400;

// let context = canvas.getContext("2d");
// let start_background_color = "gray";

// context.clearRect(0, 0, window.innerWidth, window.innerHeight);
// context.fillStyle = start_background_color;
// context.fillRect(0, 0, window.innerWidth, window.innerHeight);


// const fps_counter = wasm.FpsCounter.new()

// const renderLoop = () => {


//     //pre.textContent = universe.render();
//     fps_counter.update();

//     //断点
//     //debugger; 

//     requestAnimationFrame(renderLoop);
//   };

//   requestAnimationFrame(renderLoop);

wasm.start();  
console.debug("app started.");
