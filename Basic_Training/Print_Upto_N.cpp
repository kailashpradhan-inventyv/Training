#include<iostream>
using namespace std;
int main(){
    int n;
    cout<<"Enter N : ";
    cin>>n;
    for(int i=1;i<=n;i++){
        cout<<i<<" ";
        i++;
        cout<<-i<<" ";
    }
    return 0;
}