// import { Size } from "./size";
// import { u32 } from "./u32";

// class Base64Png {
//   constructor(public value: string) {}

//   public size(): Promise<Size> {
//     return new Promise((resolve, reject) => {
//       const img = new Image();
//       img.onload = () => {
//         const size = new Size(img.width as u32, img.height as u32);
//         resolve(size);
//       };
//       img.onerror = () => {
//         reject(new Error("image.loadFailed"));
//       };
//       img.src = this.value;
//     });
//   }
// }

// export { Base64Png };
