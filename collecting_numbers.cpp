
#include <iostream>
int main() {
	int n;
	std::cin >> n;

	int const size = 200006;
	int arrays[size];

	for (int i = 0; i < n; i++) {
		int el;
		std::cin >> el;
		arrays[el] = i;
	}

	int rotation = 1;
	for (int i = 1; i < n; i++) {
		if (arrays[i + 1] < arrays[i]) {
			rotation += 1;
		}
		
	}
	std::cout << rotation;
}
