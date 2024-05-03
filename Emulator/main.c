#include "stdio.h"
#include "stdint.h"

#define MEM_CAPACITY 4096

struct CHIP8 {
    uint8_t memory[MEM_CAPACITY];
    uint16_t pc;
    uint8_t hlt;
};

void emulate_cycle(struct CHIP8 *chip8) {
    // fetch the 2 byte instruction from memory (MSB: pc, LSB: pc+1) and store in a 16-bit unsigned int
    uint16_t instruction = (chip8->memory[chip8->pc] << 8) | chip8->memory[chip8->pc+1];
    chip8->pc += 2;

    printf("0x%04x ", chip8->pc);

    // bitmask the instruction to extract the opcode (first nibble) 
    switch (instruction & 0xF000) {
        case 0x0000:
            printf("HLT");
            chip8->hlt = 1;
            break;

        case 0x1000:
            printf("JMP");
            break;

        case 0x2000:
            printf("CALL");
            break;

        case 0x3000:
            printf("SEQ"); 
            break;

        case 0x4000:
            printf("SNE");
            break;

        case 0x6000:
            printf("SET");
            break;

        case 0x7000:
            printf("ADD");
            break;
    }

    printf(": 0x%04x\n", instruction);
}
int main(int argc, char *argv[]) {
    // exit if user hasn't specified a ROM
    if (argc < 2) {
        printf("error: no input ROM\n");
        return 1;
    }

    // initialise CHIP8 (memory and pc) values to 0
    struct CHIP8 chip8;
    chip8.pc = 0;
    for (int i = 0; i < 4096; i++)
        chip8.memory[i] = 0;

    // read binary stream from ROM into chip-8 memory
    FILE *ptr;
    ptr = fopen(argv[1], "rb");

    fread(chip8.memory, sizeof(chip8.memory), 1, ptr);

    // simulate CPU cycles
    while(chip8.hlt != 1) {
        emulate_cycle(&chip8);
    }

    return 0;
}