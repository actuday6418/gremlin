# Gremlin
Gemini browser for the terminal

# Implemented features
1. Basic text formatting according to the text/gemini specification. (Headings, sub-headings, links and more)
2. Moving up and down the browsing history.
3. Redirects from links and separate scrolling for links (Maybe this is an anti-feature?)

# Things to be completed soon
1. Improvements in UI/UX:
   1. Provide visual feedback when a background process (eg. making a request over the network) is happening.
   2. Create a help menu
   3. Provide visual feedback about what command you just used.
   
2. Low priority bugs (crash when gemini space has no links, performance issues when scrolling a lot)
3. Trust on first use (TOFU) for TLS certificates.

# Usage (All for normal mode unless mentioned otherwise)
1. Ctrl + K/L - Scroll link selector Up / Down
2. Alt + K/L - Scroll through your history
3. Up / Down - Scroll view
4. Return / Enter Key - Go to link
5. Alt + n - Enter URI to navigate to (This takes you into Edit mode)
6. Ctrl + c - Exit application (Works in Normal and Edit mode)

# Rough Screen Recording
![out](https://user-images.githubusercontent.com/56124831/115050646-9774ed80-9ef9-11eb-9f4d-5bfe6a4325d5.gif)
