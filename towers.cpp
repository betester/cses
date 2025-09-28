
#include <iostream>
#include <set>
using namespace std;
int main() {
	int n;
	cin >> n;
	multiset<int> ms;

	for (int i = 0; i < n; i++) {
		int el;
		cin >> el;
		auto it = ms.upper_bound(el);
		if (it != ms.end()) {
			ms.erase(it);
		} 	
		ms.insert(el);
	}
	cout << ms.size();
}
