/*
* @Author: edl
* @Date:	 13:11:03, 21-Jun-2020
* @Last Modified by:   edl
* @Last Modified time: 13:16:59, 21-Jun-2020
*/
public class GP5PERF {
	public static String Repeat(String item, int repeat) {
		StringBuilder thing = new StringBuilder(item);
		for (int i=0;i<repeat-1;++i) {
			thing.append(", ").append(item);
		};
		return thing.toString();
	};

	public static void main(String[] args) {
		System.out.println(Repeat("hi people!", 100000));
	};
};