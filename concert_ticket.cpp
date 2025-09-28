
#include <iostream>
#include <set>

int main() {
	int n, m;
	std::multiset<int> ticket_prices;

	std::cin >> n >> m;

	for (int i = 0; i < n; i++) {
		int el;
		std::cin >> el;
		ticket_prices.insert(el);
	}

	for (int i = 0; i < m; i++) {
		int customer_price;
		std::cin >> customer_price;

		auto result = ticket_prices.upper_bound(customer_price);
		if (result != ticket_prices.begin()) {
			--result;
			std::cout << *result << '\n';
			ticket_prices.erase(result);
		} else {
			std::cout << -1 << '\n';
		}

	}
}
