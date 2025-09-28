
#include <cstddef>
#include <iostream>
using namespace std;

// NOTE: do this later i guess, i'm bored

int main() {
	int m, n;
	cin >> n >> m;
	size_t arrays[200006];

	for (int i = 0; i < n; i++) {
		int el;
		cin >> el;
		arrays[el] = i;
	}

	int rotation = 1;
	for (int i = 1; i < n; i++) {
		if (arrays[i + 1] < arrays[i]) {
			rotation += 1;
		}
		
	}
}
