#include <Cocoa/Cocoa.h>

void platform_get_clipboard_string() {
    FILE* file = fopen("clipboard.txt", "w");
    NSPasteboard* pasteboard = [NSPasteboard generalPasteboard];
    NSString* string = [pasteboard stringForType:NSPasteboardTypeString];
    if (string) {
        const char* c_string = [string UTF8String];
        fprintf(file, "%s", c_string);
    }
    fclose(file);
}

void platform_set_clipboard_string(const char* string) {
    NSPasteboard* pasteboard = [NSPasteboard generalPasteboard];
    NSString* ns_string = [NSString stringWithUTF8String:string];
    [pasteboard clearContents];
    [pasteboard setString:ns_string forType:NSPasteboardTypeString];
}