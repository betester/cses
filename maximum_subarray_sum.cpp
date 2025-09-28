
#include <algorithm>
#include <iostream>
using namespace std;

int main() {
	int n;
	cin >> n;

	long long el;
	cin >> el;
	long long max_total = el, current_max = el;

	for (int i = 1; i < n; i++) {
		cin >> el;
		current_max = max(el, el + current_max);
		max_total = max(current_max, max_total);
	}

	cout << max_total;
}
