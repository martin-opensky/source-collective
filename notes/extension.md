# Extension Details

## Overview

The Chrome Extension will be activated when a compatible webpage is detected. Upon clicking the extension or using a keyboard shortcut, it will extract pertinent information from the page and relay it to the Tauri application.

## Approach

We are considering two methods: using Chrome's native messaging feature or setting up a TCP server within the Tauri application. Both approaches have their merits and will be evaluated accordingly.

While native messaging seems ideal, we need to consider its compatibility with other browsers like Firefox. We also need to assess if managing a TCP server would be more efficient. Therefore, both methods will be tested.

## Process

Once the Tauri application receives the data, it will store the source information, including the URL, name, and some content, in the database. It will also initiate a YouTube DLP command to download any associated files and link them to the source.

The initial steps involve creating the ability to manually add the source and run this process within the Tauri application. Subsequently, we will develop the extension for faster execution without switching from the browser to the Tauri application.

The extension should also verify that the Tauri application is running, as it is a prerequisite for the Chrome extension to function.

## References

[Chrome Native Messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging)
[Chrome Native Messaging Rust](https://docs.rs/chrome_native_messaging/latest/chrome_native_messaging/)
