#include <atomic>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <stdexcept>
#include <thread>
#include <vector>

class SpinLock {
    std::atomic_bool locked;

public:
    SpinLock()
        : locked { false }
    {
    }

    SpinLock(const SpinLock&) = delete;
    SpinLock(SpinLock&&) = delete;
    SpinLock& operator=(const SpinLock&) = delete;
    SpinLock& operator=(SpinLock&&) = delete;

    void lock()
    {
        while (locked.exchange(true, std::memory_order_acquire)) {
            std::this_thread::yield();
        }
    }

    void unlock()
    {
        locked.store(false, std::memory_order_release);
    }
};

class Guard {
public:
    explicit Guard(SpinLock& l)
        : lock { l }
    {
        lock.lock();
    }

    ~Guard()
    {
        lock.unlock();
    }

private:
    SpinLock& lock;
};

int main()
{

    SpinLock lock;
    std::int32_t t1_wins { 0 };
    std::int32_t t2_wins { 0 };
    for (auto i { 0 }; i < 100000; ++i) {
        std::vector<std::int32_t> vec;

        auto t1 = std::thread { [&]() {
            Guard g { lock };
            vec.push_back(1);
            vec.push_back(2);
        } };

        auto t2 = std::thread([&]() {
            Guard g { lock };
            vec.push_back(3);
            vec.push_back(4);
        });

        t1.join();
        t2.join();
        if (vec == std::vector<std::int32_t> { 1, 2, 3, 4 }) {
            ++t1_wins;
        } else if (vec == std::vector<std::int32_t> { 3, 4, 1, 2 }) {
            ++t2_wins;
        } else {
            std::cerr << "Unexpected result: ";
            for (auto e : vec) {
                std::cerr << e << " ";
            }
            std::cerr << '\n';
            std::abort();
        }
    }

    std::cout << "Thread 1 wins: " << t1_wins << "; Thread 2 wins: " << t2_wins << std::endl;

    return 0;
}
