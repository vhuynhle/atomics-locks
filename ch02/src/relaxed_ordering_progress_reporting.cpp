#include <atomic>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <thread>

using namespace std::chrono_literals;

void process_item(int)
{
    std::this_thread::sleep_for(50ms);
}

int main()
{
    auto num_done = std::atomic<std::uint64_t> { 0 };
    auto background_thread = std::jthread { [&]() {
        for (auto i { 0 }; i < 100; ++i) {
            process_item(i);
            num_done.store(i + 1, std::memory_order_relaxed);
        }
    } };

    while (true) {
        const auto n = num_done.load(std::memory_order_relaxed);
        std::cout << "Working ... " << n << "/100 done\n";
        if (n == 100) {
            break;
        }

        std::this_thread::sleep_for(500ms);
    }

    return 0;
}
