

#include <algorithm>
#include <iostream>

const int size = 200006;

int main() {
	int n, m, k;
	std::cin >> n >>m >> k;

	int desired_size[size];
	int apartment_size[size];

	for (int i = 0; i < n; i++) {
		int el;
		std::cin >> el;
		desired_size[i] = el;
	}

	for (int i = 0; i < m; i++) {
		int el;
		std::cin >> el;
		apartment_size[i] = el;
	}

	std::sort(desired_size, desired_size + n);
	std::sort(apartment_size, apartment_size + m);

	int i, j, total_occupied;
	while (i < n && j < m) {
		int current_desired_size = desired_size[i];
		int current_apt_size = apartment_size[j];
		if (current_desired_size - k <= current_apt_size && current_apt_size <= current_desired_size + k) {
			total_occupied += 1;
			i += 1;
			j += 1;
		} else if (current_desired_size - k > current_apt_size) {
			j += 1;
		} else {
			i += 1;
		}
	}
	std::cout << total_occupied;
	return 0;
}

