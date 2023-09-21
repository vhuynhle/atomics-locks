#include <atomic>
#include <chrono>
#include <cstdlib>
#include <iostream>
#include <string>
#include <thread>

namespace {
static auto sA = std::atomic_bool { false };
static auto sB = std::atomic_bool { false };
static auto sS = std::atomic_int32_t { 0 };
}

int main()
{
    static const auto kDelay { false };

    auto t1 = std::thread { [] {
        sA.store(true, std::memory_order_seq_cst);
        if (kDelay) {
            std::this_thread::sleep_for(std::chrono::milliseconds { 20 });
        }

        if (!sB.load(std::memory_order_seq_cst)) {
            std::cout << "Thread a: B has not been set to true, we will now access S\n";
            std::this_thread::sleep_for(std::chrono::milliseconds { 20 });
            sS += 1;
        }
    } };

    auto t2 = std::thread { [] {
        sB.store(true, std::memory_order_seq_cst);
        if (kDelay) {
            std::this_thread::sleep_for(std::chrono::milliseconds { 20 });
        }

        if (!sA.load(std::memory_order_seq_cst)) {
            std::cout << "Thread b: A has not been set to true, we will now access S\n";
            std::this_thread::sleep_for(std::chrono::milliseconds { 20 });
            sS += 1;
        }
    } };

    t1.join();
    t2.join();

    if (sS == 0) {
        std::cout << "No thread accessed S, Ok\n";
    } else if (sS == 1) {
        std::cout << "One thread accessed S, Ok\n";
    } else {
        std::cout << "More than 1 thread accessed S -> FAIL!\n";
        std::abort();
    }

    return 0;
}
