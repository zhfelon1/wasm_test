import * as wasm from "th_rust";

const pre = document.getElementById("thrust-canvas");


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
