import { readAudioBytes } from "./audioApi";

function fileNameFromPath(path: string) {
  return path.split(/[/\\]/).pop() ?? "dropped-audio";
}

export async function createAudioBlobUrl(path: string) {
  const bytes = await readAudioBytes(path);
  if (!bytes) return null;
  const file = new File([new Uint8Array(bytes)], fileNameFromPath(path), { type: "audio/*" });
  return URL.createObjectURL(file);
}
