
#include <algorithm>
#include <iostream>
#include <vector>

int main() {
	int n;
	std::vector<int> arrays;
	
	std::cin >> n;

	for (int i = 0; i < n; i++) {
		int el;
		std::cin >> el;
		arrays.push_back(el);
	}

	std::sort(arrays.begin(), arrays.end());

	int total_distinct = 0;
	int previous_num = -1;
	for (int i = 0; i < n; i++) {
		if (arrays[i] != previous_num) { 
			total_distinct += 1;
		}
		previous_num = arrays[i];
	}
	std::cout << total_distinct;
}
