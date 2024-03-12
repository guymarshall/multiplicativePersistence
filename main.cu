#include <stdio.h>
#include <stdint.h>

#include "maths.cuh"

const uint16_t NUMBERS_PER_CHUNK = 1024;
const uint16_t CUDA_CORES = 896;
const uint64_t FINISH = 277777788888899;

__global__ void calculatePersistence(uint64_t chunkStart, uint8_t* stepCounts) {
    int tid = threadIdx.x + blockIdx.x * blockDim.x;

    if (tid < NUMBERS_PER_CHUNK) {
        uint64_t currentNumber = chunkStart + tid;
        stepCounts[tid] = multiplicativePersistence(currentNumber);
    }
}

int main() {
    uint8_t highestStepsCount = 0;
    uint64_t highestStepsNumber = 0;

    const size_t size = NUMBERS_PER_CHUNK * sizeof(uint8_t);
    uint8_t* hostStepCounts = (uint8_t*) malloc(size);

    uint8_t* deviceStepCounts;
    cudaMalloc((void**) &deviceStepCounts, size);

    for (uint64_t chunkStart = 0; chunkStart <= FINISH - NUMBERS_PER_CHUNK; chunkStart += NUMBERS_PER_CHUNK) {
        calculatePersistence<<<1, CUDA_CORES>>>(chunkStart, deviceStepCounts);

        cudaMemcpy(hostStepCounts, deviceStepCounts, size, cudaMemcpyDeviceToHost);

        for (int i = 0; i < NUMBERS_PER_CHUNK; ++i) {
            uint64_t currentNumber = chunkStart + i;
            uint8_t result = hostStepCounts[i];

            if (result > highestStepsCount) {
                highestStepsCount = result;
                highestStepsNumber = currentNumber;
                printf("Up to %ld so far with %d steps\n", currentNumber, result);
            }
        }
    }

    printf("Highest step count: %ld at %d\n", highestStepsNumber, highestStepsCount);

    cudaFree(deviceStepCounts);
    free(hostStepCounts);

    return 0;
}