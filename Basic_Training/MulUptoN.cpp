#include<iostream>
using namespace std;
int main(){
    int n,res=1;
    cout<<"Enter N : ";
    cin>>n;
    for(int i=1;i<=n;i++){
        res*=i;
    }
    cout<<res;
    return 0;
}