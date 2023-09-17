#include <atomic>
#include <cstddef>
#include <cstdint>
#include <format>
#include <iostream>
#include <map>
#include <thread>
#include <utility>

static auto atomic_x = std::atomic_int32_t { 0 };
static auto atomic_y = std::atomic_int32_t { 0 };

static auto observations = std::map<std::pair<std::int32_t, std::int32_t>, std::size_t>();

void a()
{
    atomic_x.store(10, std::memory_order_relaxed);
    atomic_y.store(20, std::memory_order_relaxed);
}

void b()
{
    const auto y = atomic_y.load(std::memory_order_relaxed);
    const auto x = atomic_x.load(std::memory_order_relaxed);

    observations[std::make_pair(x, y)] += 1;
}

int main()
{
    static constexpr auto kExperiments = 1'000'000;
    for (auto i { 0 }; i < kExperiments; ++i) {
        if (i % 1000 == 0) {
            std::cout << std::format("Progress: {:8}/{} ({:.1F})%\n", i,
                kExperiments,
                static_cast<float>(i) * 100.0F / static_cast<float>(kExperiments));
        }
        atomic_x.store(0);
        atomic_y.store(0);

        std::jthread t1 { a };
        std::jthread t2 { b };
    }

    for (const auto& [k, count] : observations) {
        std::cout << std::format("({}, {}) -> {} ({:.1F}%)\n",
            k.first,
            k.second,
            count,
            static_cast<float>(count) * 100.0F / static_cast<float>(kExperiments));
    }

    return 0;
}
