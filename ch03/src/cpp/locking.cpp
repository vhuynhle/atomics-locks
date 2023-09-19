#include <atomic>
#include <iostream>
#include <string>
#include <thread>
#include <vector>

auto sData = std::string {};
auto sLocked = std::atomic_bool { false };

void f()
{
    auto false_value = bool { false };
    if (sLocked.compare_exchange_strong(false_value,
            true, std::memory_order_acquire,
            std::memory_order_relaxed)) {
        sData.push_back('!');
        sLocked.store(false, std::memory_order_release);
    }
}

int main()
{
    {
        auto handles = std::vector<std::jthread> {};
        handles.reserve(100);
        for (int i = 0; i < 100; ++i) {
            handles.emplace_back(f);
        }
    }

    std::cout << "Data: " << sData << " (length = " << sData.length() << ")\n";

    return 0;
}
