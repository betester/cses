
#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

int main() {
	int n, k;
	cin >> n >> k;
	vector<pair<int , int>> arrays;
	for (int i = 0; i < n; i++) {
		int el;
		cin >> el;
		arrays.emplace_back(el, i + 1);
	}

	sort(arrays.begin(), arrays.end());
	int i = 0, j = n - 1;
	bool found = false; 


	while (i < j) {
		int total = arrays[i].first + arrays[j].first;
		if (total == k) {
			cout << arrays[j].second << " " << arrays[i].second;
			found = true;
			break;
		} else if (total < k) {
			i += 1;
		} else {
			j -= 1;
		}
	}

	if (!found) {
		cout << "IMPOSSIBLE";
	}
}
