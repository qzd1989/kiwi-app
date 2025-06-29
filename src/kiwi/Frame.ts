import { invoke } from "@tauri-apps/api/core";
import { RgbaBuffer, u32 } from "@utils/common";

import { msgErrorObject } from "@utils/msg";

class Frame {
  width: u32;
  height: u32;
  buffer: RgbaBuffer | null;

  constructor() {
    this.width = (0).tou32();
    this.height = (0).tou32();
    this.buffer = null;
  }

  // async init() {
  //   try {
  //     const frame: unknown = await invoke("get_frame");
  //     this.width = frame?.width;
  //     this.height = frame?.height;
  //     this.data = frame?.
  //   } catch (e) {}
  // }

  // find_image(): Promise<WeightPoint | null>;
  // find_images(): Promise<WeightPoint[]>;
  // find_relative_colors(): Promise<RelativePoint | null>;
  // find_colors(): Promise<RelativePoint[]>;
  // recognize_text(): Promise<string | null>;
  // //   capture_screenshot(): Promise<Base64Png | null>;
  // get_monitor_size(): Promise<Size>;
  // get(): Promise<Frame | null>;
}

// class KiwiFrame implements Frame {
//   width: PositiveInteger = createPositiveInteger(0);
//   height: PositiveInteger = createPositiveInteger(0);
//   data: Base64Png = createBase64Png("");

//   get(): Promise<Frame | null> {
//     try {
//     } catch (e: unknown) {
//       msgErrorObject(e);
//     }
//     return Promise.resolve(null);
//   }
//   find_image(): Promise<WeightPoint | null> {
//     throw new Error("Method not implemented.");
//   }
//   find_images(): Promise<WeightPoint[]> {
//     throw new Error("Method not implemented.");
//   }
//   find_relative_colors(): Promise<RelativePoint | null> {
//     throw new Error("Method not implemented.");
//   }
//   find_colors(): Promise<RelativePoint[]> {
//     throw new Error("Method not implemented.");
//   }
//   recognize_text(): Promise<string | null> {
//     throw new Error("Method not implemented.");
//   }
//   get_monitor_size(): Promise<Size> {
//     throw new Error("Method not implemented.");
//   }
// }

// commands::frontend::frame::find_image,
// commands::frontend::frame::find_images,
// commands::frontend::frame::find_relative_colors,
// commands::frontend::frame::find_colors,
// commands::frontend::frame::recognize_text,
// commands::frontend::frame::generate_uuid,
// commands::frontend::capture::capture_screenshot,
// commands::frontend::capture::get_monitor_size,
