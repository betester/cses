
#include <algorithm>
#include <iostream>
#include <utility>
#include <vector>

int main() {
	int n;
	std::cin >> n;

	std::pair<int, int>time_ranges[200005];
	for (int i = 0; i < n; i++) {
		std::cin >> time_ranges[i].first >> time_ranges[i].second;
	}

	std::sort(time_ranges, time_ranges + n, [](const std::pair<int, int> a, const std::pair<int, int> b) {
		if (a.first == b.first) return a.second > b.second;
		return a.first < b.first;
	});

	int highest_amount_cust = 0;
	std::vector<int> heap;

	for (int i = 0; i < n; i++) {
		int last_end_duration = 0;
		if (heap.size() > 0) {
			last_end_duration = -heap[0];
		}
		while (heap.size() > 0 && last_end_duration < time_ranges[i].first) {
			std::pop_heap(heap.begin(), heap.end());
			last_end_duration = -heap[0];
			heap.pop_back();
		}
		heap.push_back(-time_ranges[i].second);
		std::push_heap(heap.begin(), heap.end());
		highest_amount_cust = std::max((int)heap.size(), highest_amount_cust);
	}

	std::cout << highest_amount_cust;
}
