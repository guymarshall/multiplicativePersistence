#include <cstdint>
#include <iostream>
#include <vector>

#include "maths.cuh"

#define LOG(input) std::cout << input << std::endl;

// __global__ void add(uint64_t *a, uint64_t *b, uint64_t *c, size_t size) {
//     uint64_t i = threadIdx.x + blockDim.x * blockIdx.x;
//     if (i < size) {
//         c[i] = a[i] + b[i];
//     }
// }

int main(void) {
    uint8_t highestStepsCount = 0;
    uint64_t highestStepsNumber = 0;
    uint64_t finish = 277777788888899;

    for (uint64_t i = 0; i <= finish; i++) {
        uint16_t result = multiplicativePersistence(i);

        if (result > highestStepsCount) {
            highestStepsCount = result;
            highestStepsNumber = i;
            std::cout << "Up to " << i << " so far with " << result << " steps" << std::endl;
        }
    }

    std::cout << "Highest step count: " << highestStepsNumber << " at " << highestStepsCount << std::endl;




    // const uint64_t LIMIT = 1024;
    // const size_t size = LIMIT * sizeof(uint64_t);

    // std::vector<uint64_t> vectorA(LIMIT);
    // std::vector<uint64_t> vectorB(LIMIT);
    // std::vector<uint64_t> vectorC(LIMIT);

    // for (uint64_t i = 0; i < LIMIT; i++) {
    //     vectorA[i] = i;
    //     vectorB[i] = LIMIT - i;
    // }

    // uint64_t *deviceA, *deviceB, *deviceC;

    // cudaMalloc(&deviceA, size);
    // cudaMalloc(&deviceB, size);
    // cudaMalloc(&deviceC, size);

    // cudaMemcpy(deviceA, vectorA.data(), size, cudaMemcpyHostToDevice);
    // cudaMemcpy(deviceB, vectorB.data(), size, cudaMemcpyHostToDevice);

    // add<<<LIMIT / 256, 256>>>(deviceA, deviceB, deviceC, LIMIT);

    // cudaDeviceSynchronize();

    // cudaMemcpy(vectorC.data(), deviceC, size, cudaMemcpyDeviceToHost);

    // cudaFree(deviceA);
    // cudaFree(deviceB);
    // cudaFree(deviceC);

    // uint64_t resultSum = 0;
    // for (uint64_t i = 0; i < LIMIT; i++) {
    //     resultSum += vectorC[i];
    // }

    // std::cout << "Result: sum = " << resultSum << std::endl;

    return 0;
}