#include <iostream>
#include <utility>
#include <algorithm>
#include <vector>
 
int main() {
	int n;
	std::cin >> n;
	std::pair<int, int>time_ranges[200005];
 
	for (int i = 0; i < n; i++) {
		std::cin >> time_ranges[i].first >> time_ranges[i].second;
	}
 
	std::sort(time_ranges, time_ranges + n, [](const std::pair<int, int> a, const std::pair<int, int> b) {
		if (a.second == b.second) return a.first > b.first;
		return a.second < b.second;
	});
	int non_overlap = 1, previous_biggest = time_ranges[0].second;
 
	for (int i = 1; i < n; i++) {
		if (previous_biggest <= time_ranges[i].first) {
			non_overlap += 1;
			previous_biggest = time_ranges[i].second;
		}
	}
 
	std:: cout << non_overlap;
}
