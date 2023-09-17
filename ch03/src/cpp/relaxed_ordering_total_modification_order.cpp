
#include <atomic>
#include <cstddef>
#include <cstdint>
#include <format>
#include <functional>
#include <iostream>
#include <thread>
#include <tuple>
#include <unordered_map>

namespace {
struct Observation {
    std::int32_t a;
    std::int32_t b;
    std::int32_t c;
    std::int32_t d;
    friend bool operator==(const Observation&, const Observation&) = default;
};

template <typename T>
std::size_t hash_combine(std::size_t seed, const T& v)
{
    std::hash<T> hasher;
    return seed ^ (hasher(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2));
}

}

template <>
struct std::hash<Observation> {
    std::size_t operator()(const Observation& o) const noexcept
    {
        const auto h1 = hash_combine(0, o.a);
        const auto h2 = hash_combine(h1, o.b);
        const auto h3 = hash_combine(h2, o.c);
        const auto h4 = hash_combine(h3, o.d);
        return h4;
    }
};

namespace {

auto atomic_x = std::atomic_int32_t { 0 };
auto observations = std::unordered_map<Observation, std::size_t> {};

void a()
{
    atomic_x.fetch_add(5, std::memory_order_relaxed);
    atomic_x.fetch_add(10, std::memory_order_relaxed);
}

void b()
{
    const auto a = atomic_x.load(std::memory_order_relaxed);
    const auto b = atomic_x.load(std::memory_order_relaxed);
    const auto c = atomic_x.load(std::memory_order_relaxed);
    const auto d = atomic_x.load(std::memory_order_relaxed);

    observations[Observation { a, b, c, d }] += 1;
}

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

        atomic_x.store(0, std::memory_order_relaxed);
        std::jthread t2 { b };
        std::jthread t1 { a };
    }

    for (const auto& [k, count] : observations) {
        std::cout << std::format("({}, {}, {}, {}) -> {} ({:.1F}%)\n",
            k.a,
            k.b,
            k.c,
            k.d,
            count,
            static_cast<float>(count) * 100.0F / static_cast<float>(kExperiments));
    }
    return 0;
}
