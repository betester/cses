
#include <algorithm>
#include <iostream>

int main() {
	int n;
	std::cin >> n;

	int arrays[200005];

	for (int i = 0; i < n; i++) {
		std::cin >> arrays[i];
	}

	std::sort(arrays, arrays + n);
	long long v = 1;
	
	// NOTE: this logic works because using induction.
	// suppose we can form a number [1, X] from a given array [a1, a2, .., an].
	// addition of an+1, would allow to form [an+1, X + an+1]. 
	// the smallest number that we couldn't form happens when a(n + 1) > X, which basically means
	// we couldn't form X + 1.
	// new range [1, X] combined with [a(n + 1), a(n + 1) + X]
	// then, suppose a(n + 1) > X, if we combine (let's say it's X+1)
	// [1 + 1 + X, X+ X + 1] -> [2 + X, X + 1], which means we couldn't form X + 1.
	for (int i = 0; i < n; i++) {
		if (arrays[i] > v) {
			break;
		}
		v += arrays[i];
	}
	std::cout << v;
}
