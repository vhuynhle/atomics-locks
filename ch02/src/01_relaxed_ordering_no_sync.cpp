#include <atomic>
#include <cstddef>
#include <format>
#include <iostream>
#include <map>
#include <thread>
#include <utility>

int main()
{
    auto x { std::atomic_int { 0 } };
    auto y { std::atomic_int { 0 } };
    std::map<std::pair<int, int>, std::size_t> occurences;

    auto f1 = [&]() -> void {
        const auto r1 { y.load(std::memory_order_relaxed) };
        x.store(r1);
    };

    auto f2 = [&]() -> void {
        const auto r2 { x.load(std::memory_order_relaxed) };
        (void)r2;
        y.store(42);
    };

    constexpr auto nExperiments { 1'000'000 };
    for (auto i = 0; i < nExperiments; ++i) {
        if (i % 1000 == 0) {
            std::cout << std::format("{:9} ({:5.2f}%)", i, static_cast<float>(i) / nExperiments * 100.0F) << '\n';
        }

        x.store(0);
        y.store(0);
        auto t2 = std::thread { f2 };
        auto t1 = std::thread { f1 };

        t1.join();
        t2.join();

        const auto key { std::make_pair(x.load(), y.load()) };
        occurences[key] += 1;
    }

    for (const auto& [k, v] : occurences) {
        std::cout << std::format("{:2} {:2} -> {:6}\n", k.first, k.second, v);
    }

    return 0;
}
