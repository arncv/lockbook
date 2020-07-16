﻿using System.Collections.ObjectModel;
using System.Linq;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;

// The Blank Page item template is documented at https://go.microsoft.com/fwlink/?LinkId=402352&clcid=0x409

namespace lockbook
{
    /// <summary>
    /// An empty page that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class Home : Page
    {
        public ObservableCollection<File> Files;
        int n;

        public Home()
        {
            InitializeComponent();
            Files = new ObservableCollection<File>();
        }

        private void AddFile(object sender, RoutedEventArgs e)
        {
            Files.Add(new File { Path = "this/is/a/file/path/" + n, Content = "This is file content " + n++ + "."});
        }

        private void FileSelected(object sender, SelectionChangedEventArgs e)
        {
            var selected = (File)e.AddedItems.Single();
            FileContentTextBlock.Text = selected.Content;
        }
    }

    public class File
    {
        public string Path { get; set; }
        public string Content { get; set; }
    }
}
