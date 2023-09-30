#include <atomic>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <thread>
#include <vector>

template <typename T>
class SpinLock;

template <typename T>
class Guard {
public:
    Guard(const Guard&) = delete;
    Guard(const Guard&&) = delete;
    Guard& operator=(const Guard&) = delete;
    Guard& operator=(const Guard&&) = delete;

    ~Guard()
    {
        lock.locked.store(false, std::memory_order_release);
    }

    T& get()
    {
        return lock.value;
    }

private:
    Guard(SpinLock<T>& l)
        : lock { l }
    {
    }

    SpinLock<T>& lock;
    friend class SpinLock<T>;
};

template <typename T>
class SpinLock {
public:
    SpinLock(T v)
        : locked { false }
        , value { v }
    {
    }

    SpinLock(const SpinLock&) = delete;
    SpinLock(SpinLock&&) = delete;
    SpinLock& operator=(const SpinLock&) = delete;
    SpinLock& operator=(SpinLock&&) = delete;

    Guard<T> lock()
    {
        while (locked.exchange(true, std::memory_order_acquire)) {
            std::this_thread::yield();
        }
        return Guard<T>(*this);
    }

private:
    std::atomic_bool locked;
    T value;
    friend class Guard<T>;
};

int main()
{
    std::int32_t t1_wins { 0 };
    std::int32_t t2_wins { 0 };
    for (auto i { 0 }; i < 100000; ++i) {
        auto v = SpinLock { std::vector<std::int32_t> {} };

        auto t1 = std::thread { [&v]() {
            auto g = v.lock();
            g.get().push_back(1);
            g.get().push_back(2);
        } };

        auto t2 = std::thread([&v]() {
            auto g = v.lock();
            g.get().push_back(3);
            g.get().push_back(4);
        });

        t1.join();
        t2.join();

        const auto& vec = v.lock().get();
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
