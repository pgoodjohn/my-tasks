import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { invoke } from "@tauri-apps/api/core"
import type { ClassValue } from "clsx";

export function cn(...inputs: Array<ClassValue>) {
  return twMerge(clsx(inputs))
}

export async function invoke_tauri_command(command: string, command_arguments: Record<string, unknown>) {

  console.debug("Invoking Tauri Command", command, command_arguments)

  try {
    const res = await invoke(command, command_arguments);
    console.debug("Tauri Returned a response for command", command, res)
    const jsonResponse = JSON.parse(res as string)
    console.debug("Parsed JSON response for command", command, jsonResponse)
    return jsonResponse

  } catch (error) {
    console.error("Tauri Command Error", command, error)
    const errorResponse = JSON.parse(error as string)
    console.debug("Parsed JSON error for command", command, errorResponse)
    throw errorResponse
  }

}