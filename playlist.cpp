
#include <algorithm>
#include <iostream>
#include <set>
#include <vector>
using namespace std;

int main() {
	int n;
	cin >> n;
	vector<int> arrays(n);
	multiset<int> ms;

	for (int i = 0; i < n; i++) {
		cin >> arrays[i];
	}

	int longest_unique = 0;

	int i = 0, j = 0;

	while (j < n) {
		auto it = ms.find(arrays[j]);
		while (it != ms.end()) {
			ms.erase(arrays[i]);
			i += 1;
			it = ms.find(arrays[j]);
		}

		ms.insert(arrays[j]);
		longest_unique = max((int)ms.size(), longest_unique);
		j += 1;
	}

	cout << longest_unique;
}
