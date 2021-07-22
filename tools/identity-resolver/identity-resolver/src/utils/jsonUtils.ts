export function formatJson(content: string|object) {
  console.log(content);

  try {
    if (typeof content === "string") {
      return JSON.stringify(JSON.parse(content), null, 4);
    } else {
      
      return JSON.stringify(content, null, 4);
    }
  } catch (err) {
    return content;
  }
}
