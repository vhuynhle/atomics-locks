#include <atomic>
#include <format>
#include <iostream>
#include <stdexcept>
#include <thread>

namespace {
std::atomic_int32_t atomic_x { 0 };

void f()
{
    const auto x = atomic_x.load(std::memory_order_relaxed);

    if (x != 1 && x != 2) {
        throw std::runtime_error("--- assertion failed ---");
    }
}

}

int main()
{
    constexpr auto kExperiments = 1'000'000;
    for (auto i { 0 }; i < kExperiments; ++i) {
        if (i % 1000 == 0) {
            std::cout << std::format("Progress: {:7}/{} ({:4.1F})\n",
                i, kExperiments,
                static_cast<float>(i * 100) / kExperiments);
        }

        atomic_x.store(1, std::memory_order_relaxed);
        atomic_x.store(2, std::memory_order_relaxed);
        auto t = std::thread { f };
        t.join();
        atomic_x.store(3, std::memory_order_relaxed);
    }

    std::cout << "All assertions satisfied\n";
    return 0;
}
