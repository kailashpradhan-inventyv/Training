#include<iostream>
using namespace std;
int main(){
    int n=4;
    for(int i=1;i<=n;i++){
        for(int j=n-1;j>=i;j--){
            cout<<" ";

        }
        for(int j=i;j<=2*i-1;j++){
            cout<<j;
        }
        for(int j=2*(i-1);j>=i;j--){
            cout<<j;
        }
        
        cout<<endl;

    }
    return 0;
}