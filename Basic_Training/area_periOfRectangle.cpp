
#include<iostream>
using namespace std;
int main(){
    float len_rec,bre_rec,area_rec,peri_rec;
    cout<<"Enter Length Of Rectangle : ";
    cin>>len_rec;
    cout<<"\nEnter Breadth Of Rectangle : ";
    cin>>bre_rec;
    area_rec=len_rec*bre_rec;
    peri_rec=2*(len_rec+bre_rec);
    cout<<"\nArea of Rectangle : "<<area_rec;
    cout<<"\nPerimeter of Rectangle : "<<peri_rec;
    return 0;
}
