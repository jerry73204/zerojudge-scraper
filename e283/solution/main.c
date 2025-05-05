#include <stdio.h>

#define BUFFER_SIZE 1024000

// Direct mapping for binary codes (faster lookup)
// The lookup table maps each binary code directly to its character
// "0101" -> 'A', "0111" -> 'B', etc.
char decode(int b1, int b2, int b3, int b4) {
    if (b1 == 0) {
        if (b2 == 1) {
            if (b3 == 0) {
                if (b4 == 1) return 'A';  // 0101
            } else { // b3 == 1
                if (b4 == 1) return 'B';  // 0111
            }
        } else { // b2 == 0
            if (b3 == 1) {
                if (b4 == 0) return 'C';  // 0010
            }
        }
    } else { // b1 == 1
        if (b2 == 1) {
            if (b3 == 0) {
                if (b4 == 1) return 'D';  // 1101
                else return 'F';          // 1100
            }
        } else { // b2 == 0
            if (b3 == 0) {
                if (b4 == 0) return 'E';  // 1000
            }
        }
    }
    return '?'; // Unknown code
}

// Fast I/O functions
char input_buffer[BUFFER_SIZE];
int input_pos = 0;
int input_size = 0;

// Read a character from the input buffer
int readChar() {
    if (input_pos >= input_size) {
        input_size = fread(input_buffer, 1, BUFFER_SIZE, stdin);
        input_pos = 0;
        if (input_size <= 0) return EOF;
    }
    return input_buffer[input_pos++];
}

// Read an integer from the input buffer
int readInt() {
    int c, n = 0;
    while ((c = readChar()) != EOF && (c < '0' || c > '9'));
    if (c == EOF) return -1;
    n = c - '0';
    while ((c = readChar()) != EOF && c >= '0' && c <= '9') {
        n = n * 10 + (c - '0');
    }
    return n;
}

char output_buffer[BUFFER_SIZE];
int output_pos = 0;

// Write a character to the output buffer
void writeChar(char c) {
    output_buffer[output_pos++] = c;
    if (output_pos >= BUFFER_SIZE) {
        fwrite(output_buffer, 1, output_pos, stdout);
        output_pos = 0;
    }
}

// Flush the output buffer
void flush() {
    if (output_pos > 0) {
        fwrite(output_buffer, 1, output_pos, stdout);
        output_pos = 0;
    }
}

int main() {
    int n;
    int first_test_case = 1;
    
    // Read until EOF
    while ((n = readInt()) != -1) {
        // Add a newline between test cases, but not before the first one
        if (!first_test_case) {
            writeChar('\n');
        }
        first_test_case = 0;
        
        // Process each character in the string
        for (int i = 0; i < n; i++) {
            // Read 4 binary digits (ignoring spaces)
            int b1, b2, b3, b4;
            char c;
            
            // Skip spaces and read binary digits
            while ((c = readChar()) == ' ' || c == '\n' || c == '\r');
            b1 = c - '0';
            
            while ((c = readChar()) == ' ' || c == '\n' || c == '\r');
            b2 = c - '0';
            
            while ((c = readChar()) == ' ' || c == '\n' || c == '\r');
            b3 = c - '0';
            
            while ((c = readChar()) == ' ' || c == '\n' || c == '\r');
            b4 = c - '0';
            
            // Decode and write the character
            writeChar(decode(b1, b2, b3, b4));
        }
    }
    
    // Flush any remaining output
    flush();
    
    return 0;
}