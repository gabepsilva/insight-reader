#!/usr/bin/env python3
"""
Simple script to detect and print selected text on macOS.
Uses Cmd+C simulation to copy selected text from any application.
"""

import pyperclip
from pynput.keyboard import Key, Controller
import time


def get_selected_text() -> str:
    """
    Get the currently selected text on macOS by simulating Cmd+C.
    
    Returns:
        str: The selected text, or empty string if nothing is selected.
    """
    # Save current clipboard content
    old_clipboard = pyperclip.paste()
    
    try:
        # Clear clipboard first to detect if something was actually copied
        pyperclip.copy("")
        
        # Simulate Cmd+C to copy selected text
        keyboard = Controller()
        keyboard.press(Key.cmd)
        keyboard.press('c')
        keyboard.release('c')
        keyboard.release(Key.cmd)
        
        # Small delay to allow clipboard to update
        time.sleep(0.1)
        
        # Get the copied text
        selected_text = pyperclip.paste()
        
        # Restore old clipboard if nothing was selected
        if not selected_text:
            pyperclip.copy(old_clipboard)
            return ""
        
        # Restore old clipboard
        pyperclip.copy(old_clipboard)
        
        return selected_text
        
    except Exception as e:
        # Restore clipboard on error
        try:
            pyperclip.copy(old_clipboard)
        except:
            pass
        print(f"Error: {e}")
        return ""


if __name__ == "__main__":
    print("Select some text in any application, then press Enter...")
    input()
    
    selected = get_selected_text()
    
    if selected:
        print("\n" + "=" * 50)
        print("Selected text:")
        print("=" * 50)
        print(selected)
        print("=" * 50)
    else:
        print("\nNo text was selected or could not be retrieved.")

