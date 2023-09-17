#include <atomic>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <thread>
#include <vector>

using namespace std::chrono_literals;

void process_item(int)
{
    std::this_thread::sleep_for(50ms);
}

int main()
{
    auto num_done = std::atomic<std::uint64_t> { 0 };
    std::vector<std::jthread> background_threads;
    for (auto t { 0 }; t < 4; ++t) {
        background_threads.emplace_back([&]() {
            for (auto i { 0 }; i < 25; ++i) {
                process_item(25 * t + i);
                num_done.fetch_add(1, std::memory_order_relaxed);
            }
        });
    }

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
