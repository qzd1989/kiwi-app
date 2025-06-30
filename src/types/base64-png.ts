type Base64Png = string;

namespace Base64Png {
  export const PREFIX = "data:image/png;base64,";

  export const from = (value: string): Base64Png => {
    if (!isValid(value)) {
      throw new Error("Invalid Base64 PNG string");
    }
    return value as Base64Png;
  };

  export const isValid = (value: string): value is Base64Png => {
    return (
      typeof value === "string" &&
      value.startsWith(PREFIX) &&
      isBase64(value.slice(PREFIX.length))
    );
  };

  const isBase64 = (data: string): boolean => {
    return /^[A-Za-z0-9+/]+={0,2}$/.test(data);
  };
}

export { Base64Png };
