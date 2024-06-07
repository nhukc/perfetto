#include <perfetto.h>
#include <iostream>
#include <thread>
#include <chrono>
#include <fstream>
#include <condition_variable>
#include <mutex>

PERFETTO_DEFINE_CATEGORIES(
    perfetto::Category("info")
        .SetDescription("Events and spans marked as info."));

PERFETTO_TRACK_EVENT_STATIC_STORAGE();

class Observer : public perfetto::TrackEventSessionObserver {
public:
    Observer() { perfetto::TrackEvent::AddSessionObserver(this); }
    ~Observer() override { perfetto::TrackEvent::RemoveSessionObserver(this); }

    void OnStart(const perfetto::DataSourceBase::StartArgs&) override {
        std::unique_lock<std::mutex> lock(mutex);
        cv.notify_one();
    }

    void WaitForTracingStart() {
        PERFETTO_LOG("Waiting for tracing to start...");
        std::unique_lock<std::mutex> lock(mutex);
        cv.wait(lock, [] { return perfetto::TrackEvent::IsEnabled(); });
        PERFETTO_LOG("Tracing started");
    }

    std::mutex mutex;
    std::condition_variable cv;
};

extern "C" {

	void initialize_tracing() {
	    perfetto::TracingInitArgs args;
	    args.backends = perfetto::kSystemBackend;
	    args.enable_system_consumer = false;

	    perfetto::Tracing::Initialize(args);
	    perfetto::TrackEvent::Register();
	}

	Observer* create_observer() {
	    return new Observer();
	}

	void wait_for_tracing_start(Observer* observer) {
	    observer->WaitForTracingStart();
	}

	void wait() {
	    TRACE_EVENT("info", "waiting");

	    for (int i = 0; i < 100000000; i++) {
		std::cout << "ABC";
	    }
	}

	void flush_tracing() {
	    perfetto::TrackEvent::Flush();
	}

	void destroy_observer(Observer* observer) {
	    delete observer;
	}

}
