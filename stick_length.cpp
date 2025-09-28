
#include <algorithm>
#include <cstdlib>
#include <iostream>
using namespace std;

int main() {
	int n;
	cin >> n;
	int arrays[200005];

	for (int i = 0; i < n; i++) {
		cin >> arrays[i];
	}

	sort(arrays, arrays + n);
	int median = 0;
	if (n % 2 == 1) {
		median = arrays[(n - 1) / 2];
	} else {
		median = arrays[n / 2];
	}

	long long total = 0;
	for (int i = 0; i < n; i++) {
		total += abs(arrays[i] - median);
	}

	cout << total;
}
