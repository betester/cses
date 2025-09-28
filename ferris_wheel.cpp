
#include <algorithm>
#include <iostream>
const int size = 200005;

int main() {
	int n, m;
	int arrays[size]; 

	std::cin >> n >> m;
	for (int i = 0; i < n; i++) {
		std::cin >> arrays[i];
	}

	std::sort(arrays, arrays + n);
	int i = 0;
	int j = n - 1;
	int gondolas_amount = 0;

	while (i <= j) {
		int total = arrays[i] + arrays[j];
		if (total > m) {
			j -= 1;
		} else {
			i += 1;
			j -= 1;
		}
		gondolas_amount += 1;
	}

	std::cout << gondolas_amount;
}
