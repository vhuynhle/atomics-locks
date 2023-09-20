#include <atomic>
#include <chrono>
#include <cstdint>
#include <ctime>
#include <iostream>
#include <ostream>
#include <syncstream>
#include <thread>
#include <vector>

struct Data {
    std::int64_t a;
    std::int64_t b;
};

std::ostream& operator<<(std::ostream& os, const Data& d)
{
    os << "{" << d.a << ", " << d.b << "}";
    return os;
}

Data* generate_data()
{
    std::this_thread::sleep_for(std::chrono::milliseconds { 20 });
    return new Data(12, 34);
}

auto get_utc_time()
{
    using namespace std::chrono;
    static const auto kUtcZonePtr = locate_zone("UTC");
    const auto now = system_clock::now();
    return zoned_time<nanoseconds, const time_zone*> { kUtcZonePtr, now };
}

Data& get_data()
{
    static auto sPtr = std::atomic<Data*> { nullptr };

    auto p = sPtr.load(std::memory_order_acquire);

    const auto tid { std::this_thread::get_id() };
    auto scout = std::osyncstream { std::cout };
    if (p != nullptr) {
        // p is not nullptr. Some thread must have initialized sPtr with memory order release
        // (see the else clause below). The release-acquire order ensures that p points to
        // initialized data, so there's nothing more for this thread to do.
        scout << get_utc_time() << ": Thread " << tid << ": uses existing data\n";
    } else {
        scout << get_utc_time() << ": Thread " << tid << ": attempts to initialize data\n";

        // Initialize the data and store it in p
        p = generate_data();

        // Try to store p in sPtr
        Data* n = nullptr;
        if (sPtr.compare_exchange_strong(n, p, std::memory_order_release,
                std::memory_order_acquire)) {
            // The compare_exchange_strong has performed a read-modify-write operation.

            scout << get_utc_time() << ": >>>>>>>> Thread " << tid << " wins the race to initialize data!\n";
        } else {
            // The compare_exchange_strong has performed a load operation.

            scout << get_utc_time() << ": Thread " << tid << " loses the race\n";
            delete p;
            p = n;
        }
    }
    scout << get_utc_time() << ": " << tid << ": " << *p << " @ " << p << '\n';

    return *p;
}

int main()
{
    std::vector<std::jthread> handles;
    handles.reserve(100);
    for (auto i { 0 }; i < 100; ++i) {
        handles.emplace_back(get_data);
        std::this_thread::sleep_for(std::chrono::milliseconds { 1 });
    }
    return 0;
}
